function animate_button(button, scale, duration, elasticity) {
    anime.remove(button);
    anime({
        targets: button,
        scale: scale,
        duration: duration,
        elasticity: elasticity
    });
}