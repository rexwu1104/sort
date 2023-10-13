import { Stack } from "react-bootstrap";

function LabeledCheckbox({ label, onChange, _value }) {
    let changer = (v) => {
        let value = v.target.checked;
        return onChange(value);
    };

    return (
        <Stack direction="horizontal" gap={2}>
            <input className="form-check-input" id="flexCheckDefault" type="checkbox" checked={_value || false} onChange={changer}></input>
            <span className="form-check-label" htmlFor="flexCheckDefault">{label}</span>
        </Stack>
    );
}

export default LabeledCheckbox;