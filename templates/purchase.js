function load_state(ledger_state) {
    var names = [];
    for (var vendor in ledger_state) {
        names.push(vendor);
        localStorage.setItem(vendor, JSON.stringify(ledger_state[vendor]));
    }
    localStorage.setItem('names', JSON.stringify(names));
}

function update_count_range(item_object) {
    let vendor = document.getElementById("from").value;
    let item = JSON.parse(localStorage.getItem(vendor))[item_object.value];
    document.getElementById("count").max = item.count;
}

function update_item_dropdown(vendor_object) {
    let items = JSON.parse(localStorage.getItem(vendor_object.value));
    var temp_select = document.createElement("select");
    for (var item in items) {
        var option = document.createElement("option");
        option.value = item;
        option.text = item;
        temp_select.appendChild(option);
    }
    document.getElementById("item").innerHTML = temp_select.innerHTML;
}