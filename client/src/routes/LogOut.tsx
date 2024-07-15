import { FormEvent, useContext, useState } from "react";
import styles from "./log-out.module.scss";
import { Context } from "../Context";
import { useLocation } from "wouter";

export default function LogIn() {
  const context = useContext(Context);
  const [, setLocation] = useLocation();
  const [attempts, setAttempts] = useState(0);
  const [response, setResponse] = useState<Response | null>(null);

  const logIn = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const res = await fetch("/api/log-out", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
    });
    if (res.status == 200) {
      context.auth.setLogin("logged-out");
      setLocation("/");
    }
    setResponse(res);
    setAttempts(attempts + 1);
  };

  const infoBarText = () => {
    switch (response?.status) {
      case 500:
        return "Unable to logout";
      default:
        return "";
    }
  };

  if (context.auth.login == "logged-out") {
    return (
      <div>
        <h1>User is already logged out</h1>
      </div>
    );
  }

  return (
    <div id={styles.page}>
      <form id={styles.form} onSubmit={logIn}>
        <h2>Logout from Patras</h2>
        <div id={styles.infoBar}>{infoBarText()}</div>
        <input id={styles.submitButton} type="submit" value="Log Out" />
      </form>
    </div>
  );
}
