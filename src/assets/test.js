import { LitElement, html } from 'https://cdn.jsdelivr.net/gh/lit/dist@3/all/lit-all.min.js'

export class TestElement extends LitElement {

  static properties = {
    counter: { state: true },
  };

  constructor() {
    super();
    this.counter = 0;
  }

  inc() {
    this.counter += 1;
  }

  render() {
    return html`
      <h1>Counter: ${this.counter}</h1>
      <button @click=${this.inc}>Click me</button>
    `;
  }

}

customElements.define('test-element', TestElement);
