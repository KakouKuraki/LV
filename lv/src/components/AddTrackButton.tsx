const AddTrackButton = ({ onClick }: { onClick: () => void }) => (
  <button
    onClick={onClick}
    className="absolute bottom-3 left-3 w-8 h-8 rounded-full bg-zinc-600 text-white text-lg font-bold flex items-center justify-center hover:bg-indigo-500 transition-colors"
  >
    +
  </button>
);

export default AddTrackButton;
