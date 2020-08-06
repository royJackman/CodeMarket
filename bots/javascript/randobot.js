const cm = require('../apis/codemarket');

class RandoBot {
    async run() {
        let ledger = {};
        let name = Math.floor(Math.random() * 100000).toString();
        let registration = await cm.register_vendor(name);
        let uuid = registration.uuid;
        while (!('error' in ledger)) {
            ledger = await cm.get_ledger_state(uuid);
            let me = ledger[name];
            let to_move = Math.floor(Math.random() * me[2].length);
            let store = ledger.stored[2][to_move];
            let stock = me[2][to_move];
        
            let to_stock = 0
            if (store > stock) {
                to_stock = Math.floor(Math.random() * store);
            } else {
                to_stock = Math.floor(Math.random() * stock);
            }
        
            await cm.stock(me[0][to_move], Math.random(), to_stock, uuid);
            await new Promise(r => setTimeout(r, 10000));
        }
    }
}

let r = new RandoBot();
r.run()
 .then(console.log(r.ledger))
 .catch((e) => console.log(e));