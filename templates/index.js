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

function renderData(name, data) {
    var datum = [
        {
            line: { color: '#00F000'},
            type: 'scatter',
            x: [...Array(data.length).keys()],
            y: data.filter(function (value) { return !Number.isNaN(value); }),
        }
    ];
    var layout = {
        title: name,
        xaxis: {
            autotick: false,
            dtick: 1,
            tick0: 0
        },
        yaxis: { 
            range: [0, 10]
        }
    };
    var d3 = Plotly.d3;
    var WIDTH_IN_PERCENT_OF_PARENT = 95,
        HEIGHT_IN_PERCENT_OF_PARENT = 70;

    var gd3 = d3.select("div[id='graph-canvas']")
    .style({
        width: WIDTH_IN_PERCENT_OF_PARENT + '%',
        'margin-left': (100 - WIDTH_IN_PERCENT_OF_PARENT) / 2 + '%',
        height: HEIGHT_IN_PERCENT_OF_PARENT + 'vh',
        'margin-bottom': (100 - HEIGHT_IN_PERCENT_OF_PARENT) / 2 + 'vh'
    });

    var my_Div = gd3.node();
            
    Plotly.newPlot(my_Div, datum, layout);
            
    window.onresize = function() { Plotly.Plots.resize( my_Div ) };
}