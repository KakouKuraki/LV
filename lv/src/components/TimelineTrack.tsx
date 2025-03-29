import TrackLabel from './TrackLabel';
import TimelineItem from './TimelineItem';
import { Track } from '../types/timeline';

const TimelineTrack = ({ track }: { track: Track }) => (
  <div className="relative flex h-5">
    {/* 背景 + 罫線ゾーン */}
    <div className="absolute left-10 top-0 w-[calc(100%-40px)] h-full z-0 bg-zinc-700
      bg-[length:20px_100%] bg-[linear-gradient(to_right,#555_1px,transparent_1px)] pointer-events-none" />

    {/* ラベル */}
    <TrackLabel label_str={track.id.toString()} labelWidth={track.labelWidth}/>

    {/* クリップ */}
    <div className="relative flex-1 h-full z-10">
      {track.items.map((item, i) => (
        <TimelineItem key={i} {...item} frameWidth={track.frameWidth} />
      ))}
    </div>
  </div>
);

export default TimelineTrack;

