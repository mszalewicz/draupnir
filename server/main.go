package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"strings"
)

func main() {
	address := ":8080"
	fs1 := http.FileServer(http.Dir("./assets"))
	http.Handle("/assets/", http.StripPrefix("/assets/", fs1))
	fs := http.FileServer(http.Dir("./static"))
	http.Handle("/static/", http.StripPrefix("/static/", fs))
	http.HandleFunc("/static/draupnir_bg.wasm", serve_wasm)
	http.HandleFunc("/", mainPage)
	http.ListenAndServe(address, nil)
}

func serve_wasm(w http.ResponseWriter, r *http.Request) {
	// Extract the file name from the URL.
	filePath := "." + r.URL.Path

	// Check if the client accepts Brotli.
	if strings.Contains(r.Header.Get("Accept-Encoding"), "br") {
		brPath := filePath + ".br"
		if _, err := os.Stat(brPath); err == nil {
			// Serve the Brotli file.
			w.Header().Set("Content-Encoding", "br")
			w.Header().Set("Content-Type", "application/wasm")
			http.ServeFile(w, r, brPath)
			return
		}
	}

	// Fall back to serving the uncompressed file.
	http.ServeFile(w, r, filePath)
}

func mainPage(writer http.ResponseWriter, request *http.Request) {

	html := `<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="stylesheet" href="static/styles.css">
    <title>Wasm Example</title>
  </head>
  <body>
  	<div class="aspect-ratio-container">
  		<canvas id="draupnir"></canvas>
    </div>
  </body>
  <script type="module">
    import init from '/static/draupnir.js'
    // init()


    // let canvas = document.querySelector("canvas");

    // if (canvas) {
    //     canvas.width = window.innerWidth;
    //     canvas.height = window.innerHeight;
    //     canvas.style.width = "100%";
    //     canvas.style.height = "100%";
    // }

    // const canvasEl = document.getElementById('draupnir');

    // let once = false;
    // const observer_callback = (_mutations, _observer) => {
    //     if (!once) {
    //     // Lock the canvas aspect ratio to prevent the fit_canvas_to_parent setting from creating a feedback loop causing the canvas to grow on resize
    //     canvasEl.style.aspectRatio = canvasEl.attributes.width.value / canvasEl.attributes.height.value;
    //     once = true;
    //     }
    // };

    // const observer = new MutationObserver(observer_callback);
    // const config = { attributeFilter: ['width', 'height'] };
    // observer.observe(canvasEl, config);

    // window.addEventListener("resize", () => {
    //     let canvas = document.querySelector("canvas");
    //     if (canvas) {
    //         canvas.width = window.innerWidth;
    //         canvas.height = window.innerHeight;
    //         canvas.style.width = "80vw";
    //         canvas.style.height = "80vh";
    //     }
    // });

    init().catch((error) => {
        if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
        throw error;
        }
    });


  </script>
</html>`

	_, err := fmt.Fprint(writer, html)
	if err != nil {
		log.Fatal(err)
	}

}
