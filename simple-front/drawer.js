const BASE_URL = "http://localhost:8000/"
const CANVAS = document.getElementById('canvas');
const CANVAS_CTX = canvas.getContext('2d');

const PIXEL_SIZE = 10;
var OPTIONS;
var COLOR_CODES;

async function getOptions() {
	const request = new Request(BASE_URL + "options");
	return fetch(request).then(response => response.json());
}

async function getColors() {
	const request = new Request(BASE_URL + "colors");
	return fetch(request).then(response => response.json());
}

async function init() {
	COLOR_CODES = await getColors(); 
	OPTIONS = await getOptions();

	CANVAS.width = (PIXEL_SIZE) * OPTIONS.width;
	CANVAS.height = (PIXEL_SIZE) * OPTIONS.height;
}

async function drawOnCanvas(array) {
	CANVAS_CTX.beginPath();

	for (let row = 0; row < OPTIONS.height; row++) {
		for (let col = 0; col < OPTIONS.width; col++) {
			const idx = row * OPTIONS.width + col;
			const color_idx = array[idx];

			CANVAS_CTX.fillStyle = COLOR_CODES[color_idx];

			CANVAS_CTX.fillRect(
				col * (PIXEL_SIZE) + 1,
				row * (PIXEL_SIZE) + 1,
				PIXEL_SIZE,
				PIXEL_SIZE
			);
		}
	}

	CANVAS_CTX.stroke();
}

async function refreshCanvas() {
	const request = new Request(BASE_URL);
	fetch(request)
		.then(resp => resp.arrayBuffer())
		.then(buffer => drawOnCanvas(new Uint8Array(buffer)));
}

(async function() {
	await init();
	await refreshCanvas();
})();
