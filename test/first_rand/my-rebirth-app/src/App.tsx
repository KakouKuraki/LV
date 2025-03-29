import { useState } from 'react';

import init, * as wasm from './wasm/rng_core'

export default function App() {
	const [num, setNum] = useState<number | null>(null);

	const handleClick = async () => {
		await init();
		const n = wasm.rand_int();
		setNum(n);
	};

	return (
	  <div className="flex flex-col items-center justify-center min-h-screen space-y-4">
	  	<h1 className="text-3x1 font-bold">Rust乱数ガチャ！</h1>
		<button
		  onClick={handleClick}
		  className="bg-pink-500 text-white px-6 py-2 rounded shadow hover:bg-pink-600"
		>
		  乱数発生！
		</button>
		{num !== null && <p className="text-x1">結果: {num}</p>}
	  </div>

    );
}
