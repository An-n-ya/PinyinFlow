import MenuIcon from '@mui/icons-material/Menu';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';

interface Props {
    children?: React.ReactElement;
    scrollTarget?: React.RefObject<HTMLDivElement | null>;
}

export default function AppHeader(props: Props) {
    return (
        <Box sx={{ flex: 'none' }}>
            <AppBar position="static">
                <Toolbar>
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
        </Box>
    );
}
