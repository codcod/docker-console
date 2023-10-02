var eventSource = new EventSource('sse');
let root = document.getElementById("root");

function createHeaderRow(headers) {
    let tr = document.createElement("tr")
    for (const h of headers) {
        let th = document.createElement("th");
        th.innerText = h;
        tr.append(th);
    }
    return tr
}

function createDataRow(obj, headers) {
    let tr = document.createElement("tr");
    for (const h of headers) {
        let td = document.createElement("td");
        td.innerText = obj[h];
        tr.appendChild(td);
    }
    return tr;
}

function createAllRows(data, headers, table) {
    let objects = data.split('\n');
    for (const obj of objects) {
        if (! obj) {
            // empty string
            return
        }
        let j = JSON.parse(obj);
        let tr = createDataRow(j, headers);
        table.appendChild(tr);
    }
}

eventSource.onmessage = function(event) {
    let headers = ["ID", "Image", "Names", "Networks", "RunningFor", "Size", "State", "Status"];
    let table = document.getElementById("table");
    let headerRow = createHeaderRow(headers);
    
    table.replaceChildren(headerRow);
    createAllRows(event.data, headers, table);
}

