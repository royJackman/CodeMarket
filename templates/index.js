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
        font: {
            color: '#CCC'
        },
        grid: true,
        paper_bgcolor: '#333',
        plot_bgcolor: '#666',
        title: {
            font: {
                size: 24
            },
            text: name.concat(' average price over time')
        },
        xaxis: {
            gridcolor: '#AAA',
            title: 'Ledger Version',
            tickcolor: '#AAA',
            tick0: 0
        },
        yaxis: { 
            gridcolor: '#AAA',
            title: 'Item Price',
            range: [0, 10],
            tickcolor: '#AAA'
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