import { useState, useEffect } from "react";
import styles from "../styles/NavHeader.module.css";
export default function NavHeader() {
  const [token, setToken] = useState<string | null>("pending");

  useEffect(() => {
    setToken(localStorage.getItem("token"));
  }, []);

  return (
    <header className={styles.Main}>
      <div className={styles.MainLeft}>
        <a href="/landing">
          <h1>Flash QC</h1>
        </a>
      </div>
      <nav className={styles.MainRight}>
        {token === null && <a href="/login">Login</a>}
        {token && token !== "pending" && <a href="/">Study</a>}
        {token && token !== "pending" && <a href="/">Library</a>}
        {token && token !== "pending" && <a href="/">Plugins</a>}
        {token && token !== "pending" && <a href="/logout">Logout</a>}
        {token && token !== "pending" && (
          <a href="/profile" className={styles.AccountLogo}>
            <img src="/circle-user.svg" />
          </a>
        )}
      </nav>
    </header>
  );
}
