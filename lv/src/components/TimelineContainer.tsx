import { useState, useEffect } from 'react';
import TimelineTracks from './TimelineTracks';
import AddTrackButton from './AddTrackButton';

const TimelineContainer = () => {
  const minFrameWidth = 4;
  const maxFrameWidth = 100;
  const [frameWidth, setFrameWidth] = useState(20);
  const [isMaxZoom, setIsMaxZoom] = useState(false);
  const labelWidth = 40;
  const item1 = { start: 2, end: 6, vstart: 2.0, vend: 6.0, label: 'item1', frameWidth: frameWidth, labelWidth: labelWidth };
  const item2 = { start: 8, end: 12, vstart: 8.0, vend: 12.0, label: 'item2', frameWidth: frameWidth, labelWidth: labelWidth };
  const track1 = {id: 1, items: [item1], frameWidth: frameWidth, isMaxZoom: isMaxZoom, labelWidth: labelWidth};
  const track2 = {id: 2, items: [item2], frameWidth: frameWidth, isMaxZoom: isMaxZoom, labelWidth: labelWidth};
  const [tracks, setTracks] = useState([ track1, track2 ]);

  const addTrack = () => {
    const newId = tracks.length + 1;
    setTracks([...tracks, { id: newId, items: [], frameWidth: frameWidth, isMaxZoom: isMaxZoom, labelWidth: labelWidth }]);
  };

  useEffect(() => {
    let isMiddleClick = false;

	const handleMouseDown = (e: MouseEvent) => {
	  if (e.button == 1) isMiddleClick = true;
	};

	const handleMouseUp = () => {
	  isMiddleClick = false;
	};

	const handleMouseMove = (e: MouseEvent) => {
	  if (isMiddleClick) {
	    setFrameWidth((prev) => {
	      const delta = e.movementY > 0 ? -2 : 2;
	      const next = Math.min(maxFrameWidth, Math.max(minFrameWidth, prev + delta));
		  if (next >= maxFrameWidth) {
		    setIsMaxZoom(true);
		  } else {
		    setIsMaxZoom(false);
		  }
	      return next;
	    });
	  }
	};

	const handleWheel = (e: WheelEvent) => {
	  e.preventDefault();
	  return;
	};

	window.addEventListener('mousedown', handleMouseDown);
	window.addEventListener('mouseup', handleMouseUp);
	window.addEventListener('wheel', handleWheel, { passive: false });
	window.addEventListener('mousemove', handleMouseMove);
    return () => {
      window.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mouseup', handleMouseUp);
      window.removeEventListener('wheel', handleWheel);
	  window.removeEventListener('mousemove', handleMouseMove);
    };
  }, []);
  return (
    <div className="w-screen h-screen bg-zinc-800 flex flex-col relative">
      <TimelineTracks tracks={tracks} frameWidth={frameWidth} isMaxZoom={isMaxZoom} labelWidth={labelWidth}/>
      <AddTrackButton onClick={addTrack} />
    </div>
  );
};

export default TimelineContainer;
