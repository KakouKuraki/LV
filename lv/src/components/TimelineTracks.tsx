import TimelineTrack from './TimelineTrack';
import { Tracks } from '../types/timeline';

const TimelineTracks = ({ tracks }: { tracks: Tracks }) => (
  <div className="flex flex-col w-full h-[calc(100%-50px)]">
    {tracks.tracks.map((track) => (
      <TimelineTrack key={track.id} id={track.id} items={track.items} frameWidth={tracks.frameWidth} isMaxZoom={tracks.isMaxZoom} labelWidth={tracks.labelWidth}/>
    ))}
  </div>
);

export default TimelineTracks;
