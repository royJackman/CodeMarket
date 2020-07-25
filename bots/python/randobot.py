import time
import pprint
import numpy as np
import os
import sys

# Add parent directory to path for api import
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import apis.codemarket as cm

# Init stuff
ledger = {}
name = str(np.random.randint(100000))
pp = pprint.PrettyPrinter(indent=4)

# Loop indefinitely and perform random actions
uuid = cm.register_vendor(name)['uuid']
while 'error' not in ledger.keys():
    ledger = cm.get_ledger_state(uuid)
    me = ledger[name]
    to_move = np.random.randint(len(me[2])) # Choose random item
    store = ledger['stored'][2][to_move]
    stock = me[2][to_move]

    # Positive for stocking, negative for storing
    if store > stock:
        to_stock = np.random.randint(store) + 1
    else:
        to_stock = -(np.random.randint(stock) + 1)

    # Create the stock request with a random price
    cm.stock(me[0][to_move], np.random.rand(), to_stock, uuid)
    time.sleep(10)

pp.pprint(ledger)
