import styles from "./page-not-found.module.scss";

const PageNotFound = () => (
  <div id={styles.page}>
    <span id={styles.titleBig}>Error 404</span>
    <span id={styles.titleSmall}>Page Not Found</span>
  </div>
);

export default PageNotFound;
