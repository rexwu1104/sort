import { useCallback, useState, useEffect } from "react";
import { Button, ButtonGroup, Dropdown, InputGroup, Offcanvas, Stack } from "react-bootstrap";
import LabeledNumberInput from "./LabeledNumberInput";
import { set, get, call } from "./utils";
import LabeledCheckbox from "./LabeledCheckbox";

let sortMethods = [
    "Bubble Sort",
    "Selection Sort",
    "Cocktail Sort",
    "Merge Sort",
    "Bucket Sort",
    "Radix Sort",
    "Insert Sort",
    "Comb Sort",
    "Shell Sort",
    "Heap Sort",
    "Quick Sort"
];

function SortPage({ openChart, chageChartData }) {
    let [, update] = useState({});
    let forceUpdate = useCallback(() => update({}), []);

    let [_n, setN] = useState(0);
    let [_range, setRange] = useState(0);
    let [_repeatition, setRepeatition] = useState(0);
    let [_step, setStep] = useState(0);
    let [_target, setTarget] = useState(0);
    let [_print, setPrint] = useState(false);
    let [_check, setCheck] = useState(false);

    let [showS, setShowS] = useState(false);
    let [select, setSelect] = useState(sortMethods[0]);
    let [randoms, setRandoms] = useState([]);
    let [sorteds, setSorteds] = useState({});
    let [sort_content, setSortContent] = useState('');
    let [searchs, setSearchs] = useState([]);
    let [search_content, setSearchContent] = useState('');
    let changeSelect = (_, event) => {
        setSelect(select = event.target.innerText);
        set('sortType', select);
        setSearchs([]);
    };

    useEffect(() => {
        async function generateSortContent() {
            let content = '';
            let echo_print = await get('echoPrint');
            let self_check = await get('selfCheck');
            let step = await get('step') || 1;
            for (let key in sorteds) {
                for (let [sorted, cpu_time] of sorteds[key]) {
                    if (echo_print) {
                        for (let i=step-1; i<sorted.length; i+=step) {
                            content += `data[${i}] = ${sorted[i]}\n`;
                        }
                    }

                    if (self_check) {
                        content += (sorted.reduce((p, c) => [p[0] && p[1] <= c, c], [true, -1])[0]
                            ? 'Sorted Correctly.'
                            : 'Sorted Failed')
                        + '\n';
                    }

                    content += `(${key}) CPU Time (sec.) ${cpu_time}\n`;
                }

                setSortContent(content);
            }
        }

        generateSortContent();
    }, [sorteds, _step, _print, _check]);

    useEffect(() => {
        async function generateSearchContent() {
            let content = '';
            for (let [index, found] of searchs) {
                content += (found
                    ? `target found at data[${index}]`
                    : 'target not found')
                + '\n';
            }

            setSearchContent(content);
        }

        generateSearchContent();
    }, [searchs]);

    function appendSorted(sorteds, sorted) {
        let new_sorteds = { ...sorteds };
        new_sorteds[select] = [...(new_sorteds[select] || []), sorted];
        return new_sorteds;
    }

    return (
        <div style={{ padding: 4, display: 'flex', flexDirection: 'column' }}>
            <Button style={{ height: 40 }} variant="outline-secondary" onClick={() => setShowS(true)}>Sort Settings</Button>
            <Stack style={{ marginTop: 8, height: window.outerHeight - 120 }} direction="horizontal" gap={2}>
                <Stack gap={2}>
                    <Button variant="outline-secondary" onClick={() => call('generate_random_numbers').then(setRandoms)}>Generate Numbers</Button>
                    <textarea style={{ flexGrow: 1, height: '100%' }} name="random-numbers" value={
                        randoms.map((v, i) => `data[${i}] = ${v}`).join('\n')
                    } disabled />
                </Stack>
                <Stack gap={2}>
                    <Button variant="outline-secondary" onClick={() => {
                        setSorteds(sorteds = { ...sorteds, [`${select}`]: [] });
                        return (async () => {
                            let n = await get('n');
                            let step = await get('step');
                            let repeatition = await get('repeatition')
                            if (step) {
                                await set('n', step);
                            }

                            let new_randoms = randoms;
                            let new_sorted = sorteds;
                            for (let _ in [...Array(repeatition || 1)]) {
                                let sorted = await call('sort');
                                if (step) {
                                    let new_r = await call('generate_random_numbers');
                                    new_randoms = [...new_randoms, ...new_r];

                                    await set('randomBuffer', new_randoms);
                                    setRandoms(new_randoms);
                                }

                                setSorteds(new_sorted = appendSorted(new_sorted, sorted));
                            }

                            await set('n', n);
                            await set('randomBuffer', new_randoms = new_randoms.slice(0, n));
                            setRandoms(new_randoms);
                        })()
                        .then(forceUpdate);
                    }}>Sort Numbers</Button>
                    <textarea style={{ flexGrow: 1, height: '100%' }} name="sort-numbers" value={sort_content} disabled />
                </Stack>
                <Stack gap={2}>
                    <Button variant="outline-secondary" onClick={async () => setSearchs(searchs = [...searchs, await call('search')])}>Search Number</Button>
                    <textarea style={{ flexGrow: 1, height: '100%' }} name="search-numbers" value={search_content} disabled />
                </Stack>
            </Stack>
            <Offcanvas style={{ width: 300 }} show={showS} onHide={() => setShowS(false)}>
                <Offcanvas.Header closeButton>
                    <Offcanvas.Title>Sort Settings</Offcanvas.Title>
                </Offcanvas.Header>
                <Offcanvas.Body>
                    <div style={{
                        display: 'flex',
                        flexDirection: 'column'
                    }}>
                        <LabeledNumberInput label="n = " onChange={(v) => (setN(v), set('n', v))} _value={_n} />
                        <LabeledNumberInput label="range = " onChange={(v) => (setRange(v), set('range', v))} _value={_range} />
                        <LabeledNumberInput label="repeatition = " onChange={(v) => (setRepeatition(v), set('repeatition', v))} _value={_repeatition} />
                        <LabeledNumberInput label="step = " onChange={(v) => (setStep(v), set('step', v))} _value={_step} />
                        <LabeledNumberInput label="target = " onChange={(v) => (setTarget(v), set('target', v))} _value={_target} />
                        <Stack direction="horizontal" gap={2}>
                            <LabeledCheckbox label="echo_print" onChange={(v) => (setPrint(v), set('echoPrint', v))} _value={_print} />
                            <LabeledCheckbox label="self_check" onChange={(v) => (setCheck(v), set('selfCheck', v))} _value={_check} />
                        </Stack>
                        <InputGroup size="sm" style={{ padding: 4, height: 40 }}>
                            <InputGroup.Text>Sort Method</InputGroup.Text>
                            <Dropdown className="form-control" as={ButtonGroup} onSelect={changeSelect}>
                                <Dropdown.Toggle variant="outline-secondary">{select}</Dropdown.Toggle>
                                <Dropdown.Menu>{
                                    sortMethods.map(v => <Dropdown.Item>{v}</Dropdown.Item>)
                                }</Dropdown.Menu>
                            </Dropdown>
                        </InputGroup>
                        <Button variant="outline-secondary" onClick={() => {
                            setShowS(false);
                            chageChartData(sorteds);
                            openChart();
                        }}>Open Chart</Button>
                    </div>
                </Offcanvas.Body>
            </Offcanvas>
        </div>
    );
}

export default SortPage;