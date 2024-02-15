import './style.css'
import { Peer, DataConnection } from 'peerjs';

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <div id="input-container">
      <input type="text" id="id-input" />
      <button id="connect-button">Connect</button>
    </div>
    <div id="status-container">
      <p>Status:</p>
      <div id="status"></div>
    </div>
    <div id="messages-container" style="display: none;">
      <div id="messages"></div>
      <div id="messages-input-container">
        <input type="text" id="messages-input" />
        <button id="send-button">Send</button>
      </div>
    </div>
  </div>
`;

const inputContainer = document.getElementById('input-container')! as HTMLDivElement;
const idInput = document.getElementById('id-input')! as HTMLInputElement;
const connectButton = document.getElementById('connect-button')! as HTMLButtonElement;
const messagesContainer = document.getElementById('messages-container')! as HTMLDivElement;
const messages = document.getElementById('messages')! as HTMLDivElement;
const status = document.getElementById('status')! as HTMLDivElement;
// const messagesInputContainer = document.getElementById('messages-input-container')! as HTMLDivElement;
const messagesInput = document.getElementById('messages-input')! as HTMLInputElement;
const sendButton = document.getElementById('send-button')! as HTMLButtonElement;

const peer = new Peer({
  host: "peerjs-test.omnia-network.com",
  debug: 3, // all
});

peer.on('open', (id) => {
  console.log('PEER', 'id:' + id);
  status.innerHTML = "ID: " + id;
});

let conn: DataConnection | null;

peer.on('connection', (c) => {
  console.log('PEER', 'connection');
  conn = c;
  // Disallow incoming connections
  conn.on('open', () => {
    console.log('CONN', 'open');
    status.innerHTML += "<br/>Connection established";
    inputContainer.style.display = 'none';
    messagesContainer.style.display = 'block';
  });

  conn.on('data', (data) => {
    console.log('CONN', 'data');
    messages.innerHTML += "<br/>" + data;
  });

  conn.on('close', () => {
    console.log('CONN', 'close');
    status.innerHTML += "<br/>Connection destroyed";
    conn = null;
    inputContainer.style.display = 'block';
    messagesContainer.style.display = 'none';
  });
});
peer.on('disconnected', () => {
  console.log('PEER', 'disconnected');
  conn = null;
  status.innerHTML = "Connection lost. Please reconnect";
  console.log('Connection lost. Please reconnect');

  peer.reconnect();
});
peer.on('close', () => {
  console.log('PEER', 'close');
  conn = null;
  status.innerHTML = "Connection destroyed. Please refresh";
  console.log('Connection destroyed');
});
peer.on('error', (err) => {
  console.log('PEER', 'error:', err);
  alert('Peerjs error:' + err);
});

connectButton.addEventListener('click', () => {
  const id = idInput.value;
  if (id) {
    peer.connect(id);
  }
});

sendButton.addEventListener('click', () => {
  if (conn) {
    conn.send(messagesInput.value);
  }
});
