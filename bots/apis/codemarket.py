import json
import requests

from typing import Optional

BASE_URL = 'http://localhost:8000'

"""CodeMarket Python API

This module is for interacting with the CodeMarket server using python. Do not
send any sensitive information through these connections.
"""

def base_api_call(url: str, data: dict) -> dict:
    """Base API call to connect with the market
    
    Args:
        url   (str):    The url to send the request to
        data (dict):    The data being sent to the endpoint

    Returns:
        dict:   Contains the response content of the call
    """
    headers = {'content-type': 'application/x-www-form-urlencoded'}
    r = requests.post(BASE_URL + url, data=data, headers=headers)
    return json.loads(r.content.decode())

def get_ledger_state(uuid: str):
    """Get the current ledger state
    
    Args:
        uuid (str):     Your UUID to verify the ledger state request

    Returns:
        dict:   Contains the current ledger state
    """
    payload = { 'uuid': uuid }
    return base_api_call('/api/ledger_state', data=payload)

def purchase(item: str, count: int, frm: str, to: str) -> dict:
    """Purchase an item FROM the vendor TO the buyer

    Args:
        item  (str):    The name of the item
        count (int):    Amount to purchase
        frm   (str):    Name of the vendor to purchase from
        to    (str):    Your UUID to verify the purchase
    
    Returns:
        dict:   Contains purchase receipt or errors
    """
    payload = {
        'item': item,
        'count': count,
        'from': frm,
        'to': to
    }
    return base_api_call('/api/purchase', data=payload)

def register_vendor(vendor_name: str, vendor_url: Optional[str] = '') -> dict:
    """Register a new vendor with the market

    Args:
        vendor_name          (str):     The name of the vendor
        vendor_url (Optional[str]):     The url of the vendor
    
    Returns:
        dict:   Contains new vendor's UUID or errors
    """
    payload = { 
        'vendor_name': vendor_name,
        'vendor_url': vendor_url
    }
    return base_api_call('/register', data=payload)

def stock(item: str, price: float, stock: int, uuid: str) -> dict:
    """Stock/store item within your a shop

    Args:
        item    (str):  The name of the item
        price (float):  Amount to purchase
        stock   (int):  The amount of items to go from the store to the stock,
                        negative values will move items from the stock to the
                        store
        uuid    (str):  Your UUID to verify the stock request
    
    Returns:
        dict:   Contains purchase receipt or errors
    """
    payload = {
        'name': item,
        'price': price,
        'stock': stock,
        'uuid': uuid
    }
    return base_api_call('/api/stock', data=payload)