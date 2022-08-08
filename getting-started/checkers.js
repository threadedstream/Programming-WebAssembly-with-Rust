document.body.onload = function () {
    fetch("./checkers.wat").then(response => 
        response.arrayBuffer()
    ).then(bytes => WebAssembly.instantiate(bytes, {
        events: {
            piecemoved: (fromX, fromY, toX, toY) => {
                console.log("A piece moved from (", fromX, ",", fromY, ") to (", toX, ",", toY, ")");
            },  
            piececrowned: (x, y) => {
                console.log("A piece at (", x, ",", y, ") has been crowned");
            }
        }
    })).then(results => {
        console.log("Loaded wasm module");
        instance = results.instance
        instance.initBoard();
        console.log("turn owner is ", instance.getTurnOwner());
        instance.exports.move(0, 5, 0, 4); ​// B​
​   	 instance.exports.move(1, 0, 1, 1); ​// W​
    ​ 	 instance.exports.move(0, 4, 0, 3); ​// B​
​ 	     instance.exports.move(1, 1, 1, 0); ​// W​
​ 	     instance.exports.move(0, 3, 0, 2); ​// B​
​ 	     instance.exports.move(1, 0, 1, 1); ​// W​
​ 	     instance.exports.move(0, 2, 0, 0); ​// B - this will get a crown​
​ 	     instance.exports.move(1, 1, 1, 0); ​// W​
​ 	      ​// B - move the crowned piece out​
​ 	      ​let​ res = instance.exports.move(0, 0, 0, 2);
    });
}
