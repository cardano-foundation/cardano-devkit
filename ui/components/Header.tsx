
const Header = ({ title }: { title: string }) => {
    return (
        <header className="w-full bg-gray-100 shadow-md h-10 flex">
            <div className="flex items-center justify-between px-4 py-4">
                <h1 className="absolute left-1/2 transform -translate-x-1/2 text-2xl font-semibold text-gray-800">
                    {title}
                </h1>
            </div>
        </header>
    );
};

export default Header;