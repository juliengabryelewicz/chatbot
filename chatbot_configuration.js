export class ChatbotConfiguration {
    constructor() {
        this._use_nlu = true;
        this._nlu_url = "http://localhost:8081/parse";
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

    get nlu_url() {
        return this._nlu_url;
    }

    set nlu_url(new_nlu_url) {
        return this._nlu_url = new_nlu_url;
    }

    get use_nlu() {
        return this._use_nlu;
    }

    set use_nlu(new_use_nlu) {
        return this._use_nlu = new_use_nlu;
    }

}
