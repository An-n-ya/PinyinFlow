import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import MenuIcon from '@mui/icons-material/Menu';
import { Slide, useScrollTrigger } from '@mui/material';

interface Props {
  children?: React.ReactElement;
  scrollTarget?: React.RefObject<HTMLDivElement | null>;
}

function HideOnScroll(props: Props) {
  const { children, scrollTarget } = props;
  const trigger = useScrollTrigger({
    target: scrollTarget?.current ?? undefined,
  });
  console.log("trigger: ", trigger, "scrollTarget: ", scrollTarget?.current)

  return (
    <Slide appear={false} direction="down" in={!trigger}>
      {children ?? <div />}
    </Slide>
  );
}


export default function AppHeader(props: Props) {
  return (
    <Box sx={{ flex: 'none' }}>
      <HideOnScroll {...props}>
        <AppBar position='static'>
          <Toolbar >
            <IconButton
              size="large"
              edge="start"
              color="inherit"
              aria-label="menu"
              sx={{ mr: 2 }}
            >
              <MenuIcon />
            </IconButton>
            <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
              对话
            </Typography>
            <Button color="inherit">登录</Button>
          </Toolbar>
        </AppBar>
      </HideOnScroll>
    </Box>
  );
}