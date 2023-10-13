import { useState } from "react";
import { InputGroup } from "react-bootstrap";

function LabeledNumberInput({ label, onChange, _value }) {
    let [value, setValue] = useState(_value || 0);
    let validator = (v) => {
        v = v.target.value;
        let vRegex = /\d*/g;
        if (vRegex.test(v) && !isNaN(v = Number(v))) {
            setValue(value = v);
        } else {
            setValue(value);
        }

        return onChange ? onChange(value) : void 0;
    };

    return (
        <InputGroup size="sm" style={{ padding: 4, height: 40 }}>
            <span className="input-group-text">{label}</span>
            <input className="form-control" placeholder="please use integer" onChange={validator} value={value}></input>
        </InputGroup>
    );
}

export default LabeledNumberInput;