import { Tab, Tabs } from "react-bootstrap";
import SortPage from "./SortPage";
import ChartPage from "./ChartPage";
import { useState } from "react";

function App() {
    let [activeKey, setKey] = useState('home');
    let [chartData, setChartData] = useState([]);
    return (
        <Tabs
            defaultActiveKey="home"
            id="uncontrolled-tab-example"
            className="mb-3"
            onSelect={(v) => setKey(v)}
            activeKey={activeKey}
        >
            <Tab eventKey="home" title="Sort Page">
                <SortPage openChart={() => setKey('chart')} chageChartData={setChartData} />
            </Tab>
            <Tab eventKey="chart" title="Chart" disabled={activeKey !== 'chart'}>
                <ChartPage data={chartData} />
            </Tab>
        </Tabs>
    );
}

export default App;