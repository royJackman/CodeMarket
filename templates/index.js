function renderGraph(type) {
    var graph_canvas = document.getElementById('graph-canvas');
    graph_canvas.value = type.value;
    var data = [
        {
            line: { color: '#00F000'},
            type: 'scatter',
            x: [1, 2, 3, 4, 5, 6, 7],
            y: [512.77, 633.56, 411.45, 300, 205.3, 706.13, 800],
        }
    ];
    var layout = {
        title: type.value,
    };
    Plotly.newPlot('graph-canvas', data, layout);
}

function renderData(data) {
    var datum = [
        {
            line: { color: '#00F000'},
            type: 'scatter',
            x: [...Array(data.length).keys()],
            y: data.filter(function (value) { return !Number.isNaN(value); }),
        }
    ];
    var layout = {
        title: 'test',
        xaxis: {
            autotick: false,
            dtick: 1,
            tick0: 0
        },
        yaxis: { 
            range: [0, 10]
        }
    };
    Plotly.newPlot('graph-canvas', datum, layout);
}