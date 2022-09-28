//@ts-ignore
import init, { greet } from 'rust';

init().then(() => {
  greet('from vite!');
});
