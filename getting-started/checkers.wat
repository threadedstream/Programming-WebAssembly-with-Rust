(module 
    (import "events" "piecemoved"
        (func $notify_piecemoved (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32)))
    
    (import "events" "piececrowned"
        (func $notify_piececrowned (param $x i32) (param $y i32)))
    (memory $mem 1)
    (global $WHITE i32 (i32.const 2))
    (global $BLACK i32 (i32.const 1))
    (global $CROWNED i32 (i32.const 4))
    (global $CURRENT_PLAYER (mut i32) (i32.const 0)) ;; player to make current move (0 - white, 1 - black)


        ;; x + (y * 8)
    (func $indexForPosition (param $x i32) (param $y i32) (result i32)
        (i32.add
            (i32.mul
                (i32.const 8)
                (local.get $y)
            )
            (local.get $x)
        )    
    )

    (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
        (i32.mul 
            (call $indexForPosition (local.get $x) (local.get $y))
            (i32.const 4)
        )
    )

    (func $isWhite (param $piece i32) (result i32)
        (i32.eq 
            (i32.and 
                (local.get $piece)
                (global.get $WHITE)
            )
            (global.get $WHITE)
        )
    )

    (func $isBlack (param $piece i32) (result i32)
        (i32.eq
            (i32.and 
                (local.get $piece)
                (global.get $BLACK)
            )
            (global.get $BLACK)
        )
    )

    (func $isCrowned (param $piece i32) (result i32)
        (i32.eq
            (i32.and 
                (local.get $piece)
                (global.get $CROWNED)
            )
            (global.get $CROWNED)
        )
        
    )

    ;; 0001 
    ;; 0010
    (func $withCrown (param $piece i32) (result i32)
        (i32.or 
            (local.get $piece)
            (global.get $CROWNED)
        )
    )

    ;; have 0101 - want 0001
    ;; have 0110 - want 0010
    (func $withoutCrown (param $piece i32) (result i32)
        (i32.and
            (local.get $piece)
            (i32.const 3)
        )
    )

    (func $setPiece (param $x i32) (param $y i32) (param $piece i32)
        (i32.store
            (call $offsetForPosition
                (local.get $x)
                (local.get $y)
            )
            (local.get $piece)
        )
    )

    (func $getPiece (param $x i32) (param $y i32) (result i32)
        (if (result i32)
            (block (result i32)
                (i32.and 
                    (call $inRange 
                        (i32.const 0)
                        (i32.const 7)
                        (local.get $x)
                    ) 
                    (call $inRange
                        (i32.const 0)
                        (i32.const 7)
                        (local.get $y)
                    )
                )
            )
        (then 
            (i32.load 
                (call $offsetForPosition 
                    (local.get $x)
                    (local.get $y)
                )
            )
        )
        (else 
            (unreachable)
        )
        ) 
    )

    (func $inRange (param $lo i32) (param $hi i32) (param $value i32) (result i32)
        (i32.and 
            (i32.ge_s
                (local.get $value)
                (local.get $lo)
            )
            (i32.le_s
                (local.get $value)
                (local.get $hi)
            )
        )
    )

    (func $shouldCrown (param $pieceY i32) (param $piece i32) (result i32)
        (i32.or 
            (i32.and 
                (i32.eq 
                    (local.get $pieceY)
                    (i32.const 0)
                )
                (call $isBlack (local.get $piece))
            )
            (i32.and 
                (i32.eq 
                    (local.get $pieceY)
                    (i32.const 7)
                )
                (call $isWhite (local.get $piece))
            )
        )
    )

    (func $crownPiece (param $x i32) (param $y i32)
        (local $piece i32)
        (local.set $piece (call $getPiece (local.get $x) (local.get $y)))

        (call $setPiece 
            (local.get $x)
            (local.get $y)
            (call $withCrown (local.get $piece))
        )

        ;; TODO(gildarov): uncomment that 
        ;; (call $notify_piececrowned (local.get $x) (local.get $y))
    )
    
    (func $distance (param $x i32) (param $y i32) (result i32)
        (i32.sub (local.get $x) (local.get $y))
    )

    (func $isValidMove (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32) (result i32)
        (local $player i32)
        (local $target i32)

        (local.set $player (call $getPiece (local.get $fromX) (local.get $fromY)))
        (local.set $target (call $getPiece (local.get $toX) (local.get $toY)))

        (if (result i32)
            (block (result i32)
                (i32.and 
                    (call $isValidJumpDistance (local.get $fromY) (local.get $toY))
                    (i32.and 
                        (call $isPlayerTurn (local.get $player))
                        (i32.eq (local.get $target) (i32.const 0))
                    )
                )
            )
            (then 
                (i32.const 1)
            )
            (else 
                (i32.const 0)
            )
        )
    )

    (func $absDistance (param $x i32) (param $y i32) (result i32)
        (if (result i32)
            (i32.gt_s (local.get $x) (local.get $y))
            (then 
                (call $distance (local.get $x) (local.get $y))
            ) 
            (else 
                (call $distance (local.get $y) (local.get $x))
            )
        )
    )

    (func $isPlayerTurn (param $player i32) (result i32)
        (i32.const 1)
    )

    (func $isValidJumpDistance (param $from i32) (param $to i32) (result i32)
        (local $distance i32)
        (local.set $distance (call $absDistance (local.get $from) (local.get $to)))
        (i32.le_u 
            (local.get $distance)
            (i32.const 2)
        )
    )

    (func $move (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32) (result i32)
        (if (result i32)
            (call $isValidMove (local.get $fromX) (local.get $fromY) (local.get $toX) (local.get $toY))
            (then 
                (call $doMove (local.get $fromX) (local.get $fromY) (local.get $toX) (local.get $toY))
            ) 
            (else 
                (i32.const 0)
            )
        )
    )
    
    (func $toggleTurnOwner
        (global.set $CURRENT_PLAYER 
            (i32.xor 
                (global.get $CURRENT_PLAYER)
                (i32.const 1)
            )
        )
    ) 

    (func $doMove (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32) (result i32)
        (local $curpiece i32)
        (local.set $curpiece (call $getPiece (local.get $fromX) (local.get $toX)))
        (call $toggleTurnOwner)
        (call $setPiece (local.get $toX) (local.get $toY) (local.get $curpiece))
        (if (call $shouldCrown (local.get $toY) (local.get $curpiece))
            (then (call $crownPiece (local.get $toX) (local.get $toY))))
        (call $notify_piecemoved (local.get $fromX) (local.get $fromY) (local.get $toX) (local.get $toY))
        (i32.const 1)
    )

    (func $setTurnOwner (param $pieceKind i32)
        (global.set $CURRENT_PLAYER (local.get $pieceKind))
    )

    (func $initBoard 
        ;; place white pieces

        ;; row 1
        (call $setPiece (i32.const 0) (i32.const 0) (global.get $WHITE))
        (call $setPiece (i32.const 2) (i32.const 0) (global.get $WHITE))
        (call $setPiece (i32.const 4) (i32.const 0) (global.get $WHITE))
        (call $setPiece (i32.const 6) (i32.const 0) (global.get $WHITE))

        ;; row 2
        (call $setPiece (i32.const 1) (i32.const 1) (global.get $WHITE))
        (call $setPiece (i32.const 3) (i32.const 1) (global.get $WHITE))
        (call $setPiece (i32.const 5) (i32.const 1) (global.get $WHITE))
        (call $setPiece (i32.const 7) (i32.const 1) (global.get $WHITE))
        
        ;; row 3
        (call $setPiece (i32.const 0) (i32.const 2) (global.get $WHITE))
        (call $setPiece (i32.const 2) (i32.const 2) (global.get $WHITE))
        (call $setPiece (i32.const 4) (i32.const 2) (global.get $WHITE))
        (call $setPiece (i32.const 6) (i32.const 2) (global.get $WHITE))

        ;; place black pieces

        ;; row 1
        (call $setPiece (i32.const 1) (i32.const 7) (global.get $BLACK))
        (call $setPiece (i32.const 3) (i32.const 7) (global.get $BLACK))
        (call $setPiece (i32.const 5) (i32.const 7) (global.get $BLACK))
        (call $setPiece (i32.const 7) (i32.const 7) (global.get $BLACK))

        ;; row 2
        (call $setPiece (i32.const 0) (i32.const 6) (global.get $BLACK))
        (call $setPiece (i32.const 2) (i32.const 6) (global.get $BLACK))
        (call $setPiece (i32.const 4) (i32.const 6) (global.get $BLACK))
        (call $setPiece (i32.const 6) (i32.const 6) (global.get $BLACK))

        ;; row 3
        (call $setPiece (i32.const 1) (i32.const 5) (global.get $BLACK))
        (call $setPiece (i32.const 3) (i32.const 5) (global.get $BLACK))
        (call $setPiece (i32.const 5) (i32.const 5) (global.get $BLACK))
        (call $setPiece (i32.const 7) (i32.const 5) (global.get $BLACK))

        (call $setTurnOwner (global.get $WHITE)) ;; white piece goes first
    )

    (export "getPiece" (func $getPiece))
    (export "isCrowned" (func $isCrowned))
    (export "getTurnOwner" (func $getTurnOwner))
    (export "initBoard" (func $initBoard))
    (export "move" (func $move))
    (export "memory" (memory $mem))
)
