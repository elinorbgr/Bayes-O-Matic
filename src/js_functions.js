export function graph_render(dot, svg) {
    var g = graphlibDot.read(dot);
    // Set margins
    g.graph().marginx = 20;
    g.graph().marginy = 20;
    // Hack: only redraw the SVG once it is actually visible,
    // otherwise firefox throws a NS_FAILURE_ERROR
    setTimeout(() => {
        d3.select(svg).call(render, g);
        // update the viewbox of svg
        var bbox = svg.getBBox();
        svg.setAttribute("viewBox", (bbox.x-10)+" "+(bbox.y-10)+" "+(bbox.width+20)+" "+(bbox.height+20));
        svg.setAttribute("width", (bbox.width+20)  + "px");
        svg.setAttribute("height",(bbox.height+20) + "px");
    }, 10);
}

export function mathjax_typeset() {
    MathJax.Hub.Queue(["Typeset",MathJax.Hub]);
}

export function make_json_download(filename, text) {
    var element = document.createElement('a');
    element.setAttribute('href', 'data:application/json,' + encodeURIComponent(text));
    element.setAttribute('download', filename);
    element.click();
  }