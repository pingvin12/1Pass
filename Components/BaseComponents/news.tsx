import React, { useState, useEffect } from "react";

export default function NewsComponent() {
  const [currentStyle, setCurrentStyle] = useState(0);
  const [news, setNews] = useState([]);

  const styles = [
    "newsletter-content1",
    "newsletter-content2",
    "newsletter-content3",
  ];

  useEffect(() => {
    // Fetch news data from an API or any data source
    // Replace the fetchData function with your actual data fetching logic
    fetchData()
      .then((data) => {
        setNews(data);
      })
      .catch((error) => {
        console.error("Failed to fetch news:", error);
      });
  }, []);

  const fetchData = () => {
    // Simulating API call with sample news data
    const sampleNews = ["News 1", "News 2", "News 3", "News 4", "News 5"];
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(sampleNews);
      }, 2000);
    });
  };

  const handlePrevClick = () => {
    setCurrentStyle((prevState) =>
      prevState === 0 ? styles.length - 1 : prevState - 1
    );
  };

  const handleNextClick = () => {
    setCurrentStyle((prevState) =>
      prevState === styles.length - 1 ? 0 : prevState + 1
    );
  };
  return (
    <div className="newsletter">
      <div className={`newsletter-content ${styles[currentStyle]}`}>
        <h2>{news[currentStyle]}</h2>
        <p>Stay up to date with the latest news and updates.</p>
        <div className="newsletter-buttons">
          <button className="newsletter-button" onClick={handlePrevClick}>
            &lt; Prev
          </button>
          <button className="newsletter-button" onClick={handleNextClick}>
            Next &gt;
          </button>
        </div>
      </div>
    </div>
  );
}
