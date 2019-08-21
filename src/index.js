const rust = import("../pkg/rust_calculator");
rust.then(m =>
    {
        const getAvgRate = (baseAsk, baseBid, counterAsk, counterBid) => {
            const base = m.div(m.add(baseAsk,baseBid),2);
            const counter = m.div(m.add(counterAsk,counterBid),2);
            return  m.div(counter,base);
        }

        for (let i = 0; i < 100; i++) {
            console.time(`getAvgRate${i}`);
            const rate = getAvgRate(0.8864693442,0.8866466558,283.79367,283.94548)
            console.timeEnd(`getAvgRate${i}`);
        }
    }
).catch(console.error);
