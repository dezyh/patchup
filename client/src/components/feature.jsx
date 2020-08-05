import React from 'react';
import Grid from '@material-ui/core/Grid';
import { makeStyles } from '@material-ui/core/styles';
import Container from '@material-ui/core/Container';
import Typography from '@material-ui/core/Typography';

const useStyles = makeStyles(theme => ({
  description: {
    margin: 0,
    padding: 0,
    width: 320,
  },
}));

const Feature = ({title, description, logo}) => {
  const classes = useStyles();
  return (
    <Grid item>
      {logo}
      <Typography variant='h4' align='center' color='textPrimary' gutterBottom>
        {title}
      </Typography>
      <Container maxWidth='xs' align='center' className={classes.description}>
        <Typography color='textSecondary' >
          {description}
        </Typography>
      </Container>
    </Grid>
  );
};

export default Feature;