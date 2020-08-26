import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import CssBaseline from '@material-ui/core/CssBaseline';
import Typography from '@material-ui/core/Typography';
import Container from '@material-ui/core/Container';
import Grid from '@material-ui/core/Grid';
import OctopusLogo from '../assets/OctopusLogo';
import SpeedLogo from '../assets/SpeedLogo';
import DeliveryLogo from '../assets/DeliveryLogo';
import Divider from '@material-ui/core/Divider';
import Feature from '../components/feature';

const useStyles = makeStyles(theme => ({
  icon: {
    marginRight: theme.spacing(2),
  },
  heroContent: {
    backgroundColor: theme.palette.background.default,
    padding: theme.spacing(8, 0, 6),
  },
  heroButtons: {
    marginTop: theme.spacing(4),
  },
  cardGrid: {
    paddingTop: theme.spacing(8),
    paddingBottom: theme.spacing(8),
  },
  card: {
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
  },
  cardMedia: {
    paddingTop: '56.25%', // 16:9
  },
  cardContent: {
    flexGrow: 1,
  },
  footer: {
    backgroundColor: theme.palette.background.paper,
    padding: theme.spacing(6),
  },
  divider: {
    margin: 35,
    width: '40%',
    minWidth: 400,
    maxWidth: 550,
  },
}));

const About = () => {
  const classes = useStyles();

  return (
    <React.Fragment>
      <CssBaseline />
      <main>
        <div className={classes.heroContent} align='center'>

          <Container maxWidth="sm" className={classes.heading}>
            <OctopusLogo height={120} width={120} centered={true}/>
            <Typography component="h1" variant="h2" align="center" color="textPrimary" gutterBottom>
              Patchup
            </Typography>
            <Typography variant="h5" align="center" color="textSecondary" paragraph>
              A service to easily manage the creation, distribution and installation of software patches.
            </Typography>
          </Container>

          <Divider variant="middle" className={classes.divider} justify='center' />

          <Container maxWidth='md'>
            <Grid container direction='row' justify='center' spacing={5}>
              <Feature 
                title='Speed' 
                description='We use a content delivery network to deliver patches as fast as possible, not matter where youre located.'
                logo={<SpeedLogo height={80} width={80} centered={true}/>}
              />
              <Feature 
                title='Delivery' 
                description='We use optimized algorithms and high performance servers to efficiently calculate your patches.'
                logo={<DeliveryLogo height={80} width={80} centered={true}/>}
              />
            </Grid>
          </Container>
          
        </div>
      </main>
    </React.Fragment>
  );
}

export default About;

// <div>Icons made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/"             title="Flaticon">www.flaticon.com</a></div>
