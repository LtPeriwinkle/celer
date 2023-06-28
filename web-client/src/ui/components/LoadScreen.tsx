//! Component for rendering a loading spinner
import "./LoadScreen.css";

/// Props for the loading component
type LoadingProps = {
    /// Color
    color: "green" | "red" | "blue" | "yellow"
}

export const LoadScreen: React.FC<LoadingProps> = ({ color }) => {
    return (
        <div className="loading-container">
            <img className="loading-logo" src={`/static/celer-${color}.svg`} />
            <div className="loading-bar" >
                <span className={color}></span>
            </div>
        </div>

    );
};
