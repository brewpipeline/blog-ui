import { initializeApp } from "https://www.gstatic.com/firebasejs/10.7.1/firebase-app.js";
import { getAnalytics, logEvent } from "https://www.gstatic.com/firebasejs/10.7.1/firebase-analytics.js";

const firebaseConfig = {
  apiKey: "AIzaSyCor6JWFh9R7qazoAth_v5eLf4AtFO-Pzs",
  authDomain: "tikitkosu.firebaseapp.com",
  projectId: "tikitkosu",
  storageBucket: "tikitkosu.appspot.com",
  messagingSenderId: "1026552961963",
  appId: "1:1026552961963:web:80cc4f5fb73fe1dbe7d0ee",
  measurementId: "G-0SGHEERGZR"
};

const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);

export function logAnalytic(event, data) {
    logEvent(analytics, event, data);
}