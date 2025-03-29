type Item = {
  start: number;
  end: number;
  vstart: number;
  vend: number;
  label: string;
  frameWidth: number;
  labelWidth?: number;
};

const TimelineItem = ({
	start,
	end,
	vstart,
	vend,
	label,
	frameWidth,
	labelWidth = 40,
}: Item) => {
  const left = start * frameWidth + labelWidth;
  const width = (end - start) * frameWidth;

  return (
    <div
      className="absolute top-0 h-full bg-zinc-600 text-white text-xs flex items-center rounded-sm pl-1 pr-2 overflow-hidden whitespace-nowrap text-ellipsis hover:bg-indigo-500 z-10"
      style={{
        left: `${left}px`,
        width: `${width}px`,
      }}
    >
      {label}
      <div className="absolute bottom-0 left-0 w-full h-[3px] rounded-b-sm bg-gradient-to-r from-orange-400 to-pink-600" />
    </div>
  );
};

export default TimelineItem;
