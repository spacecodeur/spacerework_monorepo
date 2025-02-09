import { JSX, useState } from 'react';

interface ApiResponse {
	id?: number;
	title?: string;
	body?: string;
	userId?: number;
	error?: string;
}

export default function IndexPage(): JSX.Element {
	const [response, setResponse] = useState<ApiResponse | null>(null);

	const handleGet = async (): Promise<void> => {
		try {
			const res = await fetch('http://0.0.0.0:3000/api/example_queries');
			const data: ApiResponse = await res.json();
			setResponse(data);
		} catch (error) {
			console.error('GET request failed:', error);
			setResponse({ error: 'GET request failed' });
		}
	};

	const handlePost = async (): Promise<void> => {
		try {
			const res = await fetch('http://0.0.0.0:3000/api/example_queries', {
				method: 'POST',
				headers: { 
					'Content-Type': 'application/json',
					'body' : JSON.stringify({ title: 'Test' })
				},
				// body: JSON.stringify({ title: 'Test' }) // tuono ne supporte pas encore le passage par body
			});
			const data: ApiResponse = await res.json();
			setResponse(data);
		} catch (error) {
			console.error('POST request failed:', error);
			setResponse({ error: 'POST request failed' });
		}
	};

	const handlePut = async (): Promise<void> => {
		try {
			const res = await fetch('https://jsonplaceholder.typicode.com/posts/1', {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ title: 'Updated Test' })
			});
			const data: ApiResponse = await res.json();
			setResponse(data);
		} catch (error) {
			console.error('PUT request failed:', error);
			setResponse({ error: 'PUT request failed' });
		}
	};

	const handleDelete = async (): Promise<void> => {
		try {
			const res = await fetch('https://jsonplaceholder.typicode.com/posts/1', { method: 'DELETE' });
			const data: ApiResponse = await res.json();
			setResponse(data);
		} catch (error) {
			console.error('DELETE request failed:', error);
			setResponse({ error: 'DELETE request failed' });
		}
	};

	return (
		<div>
			<button type="button" onClick={handleGet}>GET</button>
			<button type="button" onClick={handlePost}>POST</button>
			<button type="button" onClick={handlePut}>PUT</button>
			<button type="button" onClick={handleDelete}>DELETE</button>
			{response && <pre>{JSON.stringify(response, null, 2)}</pre>}
		</div>
	);
}
