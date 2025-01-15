import { GreeterClient } from './protos/generated/HelloworldServiceClientPb'
import { HelloRequest } from './protos/generated/helloworld_pb'
import * as grpcWeb from 'grpc-web';

window.addEventListener('DOMContentLoaded', () => {
  const sendRequestButton = document.getElementById('sendRequestButton') as HTMLButtonElement;
  const serverUrlInput = document.getElementById('serverUrl') as HTMLInputElement;
  const responseContainer = document.getElementById('response') as HTMLPreElement;

  // Click event to send the gRPC request
  sendRequestButton.addEventListener('click', () => {
    let serverUrl = serverUrlInput.value.trim();

    if (!serverUrl) {
      alert('Please enter a valid server URL.');
      return;
    }

    if(!serverUrl.startsWith("http")) serverUrl = "http://" + serverUrl;

    const client = new GreeterClient(serverUrl);
    const request = new HelloRequest();

    const metadata = {} as grpcWeb.Metadata;

    client.sayHello(request, metadata, (err, response) => {
      if (err) {
        console.error('Error calling gRPC method:', err);
        responseContainer.textContent = `Error: ${err.message || 'Unknown error'}`;
      } else {
        console.log('gRPC Response:', response);
        responseContainer.textContent = JSON.stringify(response.toObject(), null, 2); // Display response as JSON
      }
    });
  });
});
