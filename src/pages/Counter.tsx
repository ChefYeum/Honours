import { Box, Button, ButtonGroup, useTheme } from "@mui/material";

function Counter(props: { count: number; plus: () => void; minus: () => void }) {
  const theme = useTheme();

  const { count, plus, minus } = props;

  const buttonWidth = "calc(33.33% - 1px)";

  return (
    <div
      style={{
        textAlign: "center",
        color: theme.palette.primary.main,
        border: ".1px solid",
        borderRadius: '15px 50px 30px'
      }}
    >
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          height: "100%",
        }}
      >
        <Button
          sx={{
            width: buttonWidth,
            color: theme.palette.primary.main,
            fontWeight: "bold",
          }}
          onClick={minus}
        >
          -
        </Button>
        <Box
          component="span"
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            width: buttonWidth,
          }}
        >
          {count}
        </Box>
        <Button
          sx={{
            width: buttonWidth,
            color: theme.palette.primary.main,
            fontWeight: "bold",
          }}
          onClick={plus}
        >
          +
        </Button>
      </div>
    </div>
  );
}

export default Counter;
