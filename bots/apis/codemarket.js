/** 
 * CodeMarket JavaScript API
 * 
 * This module is for interacting with the CodeMarket server using javascript.
 * Do not send any sensitive information through these connections.
 */

const BASE_URL = 'http://[::1]:8000'; // JS localhost url

/**
 * Base API GET request to get data from the market
 * @param {string} url - The request extension
 * @returns {object} The return object
 */
exports.base_api_get = async function(url) {
    const got = require('got');

    try {
        const response = await got(BASE_URL.concat(url));
        return JSON.parse(response.body);
    } catch (error) {
        return JSON.parse(error.response.body);
    }
};

/**
 * Base API POST request to change data in the market
 * @param {string} url - The request extension
 * @param {object} data - The data being sent to the endpoint
 * @returns {object} The return object
 */
exports.base_api_post = async function(url, data) {
    const got = require('got');

    try {
        const response = await got.post(BASE_URL.concat(url), {form: data});
        return JSON.parse(response.body);
    } catch (error) {
        return JSON.parse(error.response.body);
    }
};

/**
 * Get the current ledger state
 * @param {string} uuid - Your UUID for verification
 * @returns {object} The current ledger state
 */
exports.get_ledger_state = async function(uuid) {
    payload = { uuid };
    return await exports.base_api_post('/api/ledger_state', payload);
};

/**
 * Get a list of registered vendors
 * @returns {object} A list of currently registered vendors
 */
exports.get_vendor_names = async function() {
    return await exports.base_api_get('/api/vendor_names');
};

/**
 * Get a list of vendor urls
 * @returns {object} A list of urls for all registered vendors
 */
exports.get_vendor_urls = async function() {
    return await exports.base_api_get('/api/vendor_urls');
};

/**
 * Purchase an item FROM the vendor TO the buyer
 * @param {string} item - The name of the item
 * @param {integer} count - Amount to purchase
 * @param {string} frm - Name of the vendor to purchase from
 * @param {string} to - Your UUID to verify the purchase
 * @returns {object} Contains receipt or errors
 */
exports.purchase = async function(item, count, frm, to) {
    payload = { item, count, frm, to };
    return await exports.base_api_post('/api/purchase', payload);
};

/**
 * Register a new vendor with the market
 * @param {string} vendor_name - The name of the vendor
 * @param {string} vendor_url - Optional custom url for vendor
 * @returns {object} Contains new UUID or errors
 */
exports.register_vendor = async function(vendor_name, vendor_url = '') {
    payload = { vendor_name, vendor_url };
    return await exports.base_api_post('/register', payload);
};

/**
 * 
 * @param {string} name - The name of the item
 * @param {float} price - Amount to set price of item
 * @param {integer} stock - The amount of items to go from the store to the 
 *                          stock, negative values will move items from the 
 *                          stock to the store
 * @param {string} uuid - Your UUID to verify the stock request
 * @returns {object} Contains stocking receipt or errors
 */
exports.stock = async function(name, price, stock, uuid) {
    payload = {name, price, stock, uuid};
    return await exports.base_api_post('/api/stock', payload);
};