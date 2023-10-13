import { useState } from "react";
import { Button, Dropdown, Form, Offcanvas } from "react-bootstrap";
import Plot from "react-plotly.js";

function ChartPage({ data }) {
    let [showC, setShowC] = useState(false);
    let [check, setCheck] = useState([]);
    let types = [["line", "lines"], ["bar", "lines"], ["scatter", "markers"]];

    let plot_data = [];
    let checked = check.map((v, i) => v ? i : -1).filter(v => v >= 0);
    for (let index of checked) {
        let [type, mode] = types[index];
        for (let key in data) {
            let trace = {
                type,
                mode,
                name: key
            };

            trace.x = data[key].map(v => v[0].length);
            trace.y = data[key].map(v => v[1]);

            plot_data.push(trace);
        }
    }
    
    return (
        <div style={{ padding: 4, display: 'flex', flexDirection: 'column' }}>
            <Button variant="outline-secondary" onClick={() => setShowC(true)}>Chart Settings</Button>
            <div style={{ display: 'flex', flexDirection: 'row' }}>
                <Plot
                    data={plot_data}
                    layout={{
                        width: window.outerWidth - 20,
                        height: window.outerHeight - 110
                    }}
                ></Plot>
            </div>
            <Offcanvas style={{ width: 300 }} show={showC} onHide={() => setShowC(false)}>
                <Offcanvas.Header closeButton>
                    <Offcanvas.Title>Chart Settings</Offcanvas.Title>
                </Offcanvas.Header>
                <Offcanvas.Body>
                    <Form>
                        <div key="default-checkbox" className="mb-3">{
                            types.map((v, i) => <Form.Check
                                type="checkbox"
                                label={v[0]}
                                checked={check[i]}
                                onChange={(e) => {
                                    check[i] = e.target.checked;
                                    setCheck([...check]);
                                }}
                            />)
                        }</div>
                    </Form>
                </Offcanvas.Body>
            </Offcanvas>
        </div>
    );
}

export default ChartPage;