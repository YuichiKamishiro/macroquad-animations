# How to use
```rust
    // Load spritesheet
    animator.load("attack.png").await;

    // Creating vec of frames
    let frames = vec![
        (Rect::new(0., 0., 120., 80.), 0.5),
        (Rect::new(120., 0., 120., 80.), 0.1),
        (Rect::new(240., 0., 120., 80.), 0.1),
        (Rect::new(360., 0., 120., 80.), 0.3),
    ];

    // Add frames to queue
    animator.add_frames(frames);
    
    loop {
        clear_background(BLACK);

        // Updating state
        animator.update();

        // Draw sprite
        animator.draw(200., 200.);


        next_frame().await
    }
```