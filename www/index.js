import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();
universe.set_random_cells();

let animationId = null;



const isPaused = () => {
    return animationId === null;
}

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
      play();
    } else {
      pause();
    }
  });
  
const renderLoop = () => {
    // debugger; //специальная программная точка останова в js, закоментил, чтобы не напрягала. В случае чего разкомментить. 
    pre.textContent = universe.render();
    universe.tick();
    animationId = requestAnimationFrame(renderLoop); 
};

play();

