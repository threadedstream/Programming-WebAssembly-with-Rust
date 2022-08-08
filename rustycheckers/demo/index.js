document.onload = () => {
    fetch('../target/wasm32-unknown-unknown/debug/rustycheckers.wasm').then(response => 
        response.arrayBuffer()
    ).then(bytes => WebAssembly.instantiate(bytes, {
        env: {
            notify_piecemoved: (fx, fy, tx, ty) => {
                console.log('A piece moved from (', fx, ',', fy, ') to (', tx, ',', ty, ')');
            },
            notify_piececrowned: (x, y) => {
                console.log('A piece crowned at (', x, ',', y, ')');
            }
        },
    })).then(results => {
        instance = results.instance
        console.log('At start, current turn is ', instance.exports.get_current_turn());
        let piece = instance.exports.get_piece(0, 7);
        console.log('Piece at (0, 7) is ', piece);
        
        let res = instance.exports.move_piece(0, 5, 1, 4);
        console.log("First move result: ", res);
        console.log("Turn after move: ", instance.exports.get_current_turn());

        let bad = instance.exports.move_piece(1, 4, 2, 3);
        console.log('Illegal move result: ', bad);
        console.log('Turn after illegal move: ', instance.exports.get_current_turn());

    }).catch(console.error);
}