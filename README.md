# CodeMarket v0.1

![Last Commit](https://img.shields.io/github/last-commit/royJackman/CodeMarket) ![Top Language](https://img.shields.io/github/languages/top/royJackman/CodeMarket) ![Languages](https://img.shields.io/github/languages/count/royJackman/CodeMarket) ![Size](https://img.shields.io/github/repo-size/royJackman/CodeMarket)

## Welcome to the Code Market!

This is a platform form building and teaching bots to sell goods and outsmart their opponents in this game of bets, calls, trades, and a whole lot of other words that mean you get the upper hand on your opponent! Each game is a spawned rust server that can run anywhere, and ends when that server is shut down.

## How to Play

1. Register your node at ```/register``` and *do not lose the response uuid*, it is only ever sent once and there is no way of recovering it
2. Get the current state of the market from the ledger at ```/api/ledger_state``` to help you decide how to prive your items
3. Use ```/api/stock``` to change the price or stock the shelves
4. Use ```/api/purchase``` to buy stocked goods from other vendors
5. Have the most bits at the end of the session!

## Supported Languages

Currently, CodeMarket apis have been created for
* [JavaScript](https://github.com/royJackman/CodeMarket/blob/master/bots/apis/codemarket.js)
* [Python](https://github.com/royJackman/CodeMarket/blob/master/bots/apis/codemarket.py)

## Running a local instance

CodeMarket runs on a [Rocket](rocket.rs) server with minimal [Tera templates](https://tera.netlify.app/docs/) for manual user interaction. To run the server on default port 8000, use ```cargo run``` in the CodeMarket folder.

## Running tests

Tests are in [test.rs](https://github.com/royJackman/CodeMarket/blob/master/src/tests.rs) and can be run using ```cargo test```.