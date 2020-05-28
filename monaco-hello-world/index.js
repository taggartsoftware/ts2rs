window.onload = function () {
    require(['vs/editor/editor.main'], function () {
        console.log("loaded editor");
        monaco.editor.create(document.getElementById("editor"), {
            value: "function hello() {\n\talert('Hello world!');\n}",
            language: "javascript"
        });
    });
};

