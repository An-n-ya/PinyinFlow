import "./App.css";
import Chat from "./components/chat/Chat";
import AppHeader from "./components/Header";
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
const darkTheme = createTheme({
  palette: {
    mode: 'dark',
  },
});


function App() {
  return (
    <ThemeProvider theme={darkTheme}>
    <CssBaseline/>
    <main className="container">
      <AppHeader/>
      <Chat/>
    </main>
    </ThemeProvider>
  );
}

export default App;

