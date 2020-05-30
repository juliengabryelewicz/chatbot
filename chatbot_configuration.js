export class ChatbotConfiguration {
    constructor() {
        this._title = "This is my chatbot";
        this._typing_text = "Robot is typing....";
    }

    get title() {
        return this._title;
    }

    set title(new_title) {
        return this._title = new_title;
    }

    get typing_text() {
        return this._typing_text;
    }

    set typing_text(new_typing_text) {
        return this._typing_text = new_typing_text;
    }

}
