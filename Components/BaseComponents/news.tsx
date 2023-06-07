import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface NewsItem {
  title: string,
  content: string
}

export default function NewsComponent() {
  const [currentStyle, setCurrentStyle] = useState(0);
  const [news, setNews] = useState<NewsItem[]>([]);

  const styles = [
    "newsletter-content1",
    "newsletter-content2",
    "newsletter-content3",
  ];

  useEffect(() => {
    // Fetch news data from an API or any data source
    // Replace the fetchData function with your actual data fetching logic
    fetchData()
      .then((data: NewsItem[]) => {
        setNews(data);
      })
      .catch((error) => {
        console.error("Failed to fetch news:", error);
      });
  }, []);

  const fetchData = () => {
    // Simulating API call with sample news data
    const sampleNews: NewsItem[] = [
      { title: "News 1", content: "Content 1" },
      { title: "News 2", content: "Content 2" },
      { title: "News 3", content: "Content 3" },
      { title: "News 4", content: "Content 4" },
      { title: "News 5", content: "Content 5" },
    ];
    return new Promise<NewsItem[]>((resolve) => {
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
        {news.length > 0 ? (
          <>
            <h2>{news[currentStyle].title}</h2>
            <p>{news[currentStyle].content}</p>
            <div className="newsletter-buttons">
              <button className="newsletter-button" onClick={handlePrevClick}>
                &lt; Prev
              </button>
              <button className="newsletter-button" onClick={handleNextClick}>
                Next &gt;
              </button>
            </div>
          </>
        ) : (
          <p>Loading news...</p>
        )}
      </div>
    </div>
  );
  
}
