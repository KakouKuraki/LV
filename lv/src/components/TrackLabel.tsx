import { Label } from '../types/timeline';

const TrackLabel = ({ label } : { label : Label }) => (
  <button className="w-[{label.labelWidth}px] h-full bg-zinc-600 text-white text-xs flex items-center justify-center z-10">
    { label.label }
  </button>
);

export default TrackLabel;
