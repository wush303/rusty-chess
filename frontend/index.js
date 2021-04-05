"use strict";
//const text = document.getElementById('text');
const uri = 'wss://' + location.host + '/game';
const ws = new WebSocket(uri);

let board = null
let game = new Chess()
let $status = $('#status')
let $fen = $('#fen')
let $pgn = $('#pgn')
let color = ""
let resigned = null
let win = false

function onDragStart (source, piece, position, orientation) {
    // do not pick up pieces if the game is over
    if (game.game_over()) return false

    if (game.turn() !== color) return false
    // only pick up pieces for the side to move
    if ((game.turn() === 'w' && piece.search(/^b/) !== -1) || (game.turn() === 'b' && piece.search(/^w/) !== -1)) {
        return false
    }
    //if ((game.turn() === color && piece.search(/^b/) !== -1) ||
    //    (game.turn() === 'b' && piece.search(/^w/) !== -1)) {
    //  return false
    //}

}
function onDrop (source, target) {
    // see if the move is legal
    let move = game.move({
    from: source,
    to: target,
    promotion: 'q' // NOTE: always promote to a queen for example simplicity
    })

    // illegal move
    if (move === null) return 'snapback'

    ws.send('{"MovePiece":["' + source + '", "' + target + '"]}')
    updateStatus()
}

// update the board position after the piece snap
// for castling, en passant, pawn promotion
function onSnapEnd () {
  board.position(game.fen())
}

function updateStatus () {
    var status = ''

    var moveColor = 'White'
    if (game.turn() === 'b') {
        moveColor = 'Black'
    }

    if (win == true) {
        status = "You won. Reload to join new game"
    }
    // checkmate?
    else if (game.in_checkmate()) {
        status = 'Game over, ' + moveColor + ' is in checkmate. Reload to join new game'
    }

    // draw?
    else if (game.in_draw()) {
        status = 'Game over, drawn position'
    }

    else if (resigned !== null) {
        let winner = "White"
        if (resigned == "w") winner = "Black"
        status = resigned + ' has resigned. ' + winner + ' is the winner.'
    }

    // game still on
    else {
        if (color != game.turn()) status = moveColor + ' to move'
        else status = 'Your turn' 

        

        // check?
        if (game.in_check()) {
            status += ', ' + moveColor + ' is in check'
        }
    }

  $status.html(status)
}

var config = {
  draggable: true,
  position: 'start',
  onDragStart: onDragStart,
  onDrop: onDrop,
  onSnapEnd: onSnapEnd
}
board = Chessboard('myBoard', config)

updateStatus()

ws.onopen = function() {
    chat.innerHTML = '<p><em>Finding game...</em></p>';
};
ws.onmessage = function(msg) {

    let obj = JSON.parse(msg.data)
    for (var key in obj) {
        if (key == "Hello") {
            chat.getElementsByTagName('em')[0].innerText = 'In game...';
            color = obj.Hello
            if (obj.Hello == "b") {
                board.flip();
            }
            updateStatus()
        } else if (key == "NewMove") {

            let source = obj.NewMove.f
            let target = obj.NewMove.t

            game.move({
                from: source,
                to: target,
                promotion: 'q' // NOTE: always promote to a queen for example simplicity
            })
            updateStatus()
            board.position(game.fen())

        } else if (key == "Resign") {
            console.log("resign")
            resigned = "White" 
            updateStatus()
        } else if (obj == "Win") {
            console.log("winner")
            win = true
            updateStatus()
        } else  {
            console.log("no match: " + msg.data)
        }
    }

    
};
ws.onclose = function() {
    chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
};

