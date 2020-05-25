import init from "./pkg/mazeweb.js"

const runWasm = async () => {
    const mazeweb = await init("./pkg/mazeweb_bg.wasm");

    const wasmByteMemoryArray = new Uint8Array(mazeweb.memory.buffer);

    // Get our canvas element from our index.html
    const canvasElement = document.querySelector("canvas");

    // Set up Context and ImageData on the canvas
    const canvasContext = canvasElement.getContext("2d");
    const canvasImageData = canvasContext.createImageData(
	canvasElement.width,
	canvasElement.height
    );

    // Clear the canvas
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);


    const drawBoard = (px, py, angle) => {
	const boardX = 1080;
	const boardY = 768;

	mazeweb.generate_board(px, py, angle);

	const wasmByteMemoryArray = new Uint8Array(mazeweb.memory.buffer);

	const outputPointer = mazeweb.get_output_buffer_pointer();
	const imageDataArray = wasmByteMemoryArray.slice(
	    outputPointer,
	    outputPointer + boardX * boardY * 4
	);

	canvasImageData.data.set(imageDataArray);

	canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

	canvasContext.putImageData(canvasImageData, 0, 0);
    };

    var x = 150;
    var y = 150;
    var angle = 0;

    var t = 0;
    var angle_z = 0;
    setInterval(() => {
	drawBoard(x, y, angle, angle_z);
	t+= 1
	angle = angle + 0.05*t;
	angle_z = Math.sin(0.05*t);
    }, 1000);
};

runWasm();
