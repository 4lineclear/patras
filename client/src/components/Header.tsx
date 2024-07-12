import { Link } from "wouter";
import { Context } from "../Context";
import { useContext } from "react";
import styles from "./header.module.scss";

/**
 * The optional login buttons
 */
function Buttons() {
  return (
    <div>
      <span className={styles.linkContainer}>
        <Link href="log-in">
          <span className={styles.linkText}>Log In</span>
        </Link>
      </span>
      <span className={styles.linkContainer}>
        <Link href="sign-up">
          <span className={styles.linkText}>Sign Up</span>
        </Link>
      </span>
    </div>
  );
}

export default function Header() {
  const context = useContext(Context);
  return (
    <header>
      <Link id={styles.titleHolder} href="/">
        <span className={styles.titleLarge}>Pa</span>
        <span className={styles.titleSmall}>per</span>
        <span className={styles.titleLarge}>Tra</span>
        <span className={styles.titleSmall}>der</span>
        <span className={styles.titleLarge}>S</span>
        <span className={styles.titleSmall}>olo</span>
      </Link>
      {context.auth.login ? null : <Buttons />}
    </header>
  );
}
