import * as _ from "lodash";
import moment from "moment";
import { PlotData } from "plotly.js";
import { Component } from "react";
import * as React from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import Plot from "react-plotly.js";
import { connect } from "react-redux";
import { Label } from "reactstrap";
import {
  changeNetworkOutAllAVGFilters,
  fetchNetworkOutAllAVGGraph
} from "../../../actions/graph";
import { ReducerConfigure } from "../../../reducers";
import { GraphNetworkOutAllAVGRow } from "../../../requests/types";
import "./NetworkOutAllAVGGraph.css";

interface OwnProps {
  history: any;
}

interface StateProps {
  fromTime: number;
  toTime: number;
  data: GraphNetworkOutAllAVGRow[];
}

interface DispatchProps {
  dispatch: any;
}

type Props = StateProps & DispatchProps & OwnProps;
class NetworkOutAllAVGGraph extends Component<Props> {
  public constructor(props: any) {
    super(props);
  }

  public componentDidMount(): void {
    this.props.dispatch(fetchNetworkOutAllAVGGraph());
  }

  public render() {
    const { fromTime, toTime } = this.props;
    const rowsByNodeName = _.groupBy(this.props.data, row => row.nodeName);
    return (
      <div className="network-out-all-avg-graph">
        <div className="from-time">
          <Label className="form-check-label" for="from-time-check">
            From
          </Label>
          <DatePicker
            className="date-picker"
            selected={moment.unix(fromTime)}
            onChange={this.handleChangeFromTime}
            onChangeRaw={this.handleChangeFromTimeRawDate}
            timeIntervals={10}
            showTimeSelect={true}
            dateFormat="YYYY-MM-DD HH:mm:ssZ"
          />
        </div>
        <div className="to-time">
          <Label className="form-check-label" for="from-time-check">
            To
          </Label>
          <DatePicker
            className="date-picker"
            selected={moment.unix(toTime)}
            onChange={this.handleChangeToTime}
            onChangeRaw={this.handleChangeToTimeRawDate}
            timeIntervals={10}
            showTimeSelect={true}
            dateFormat="YYYY-MM-DD HH:mm:ssZ"
          />
        </div>
        <div className="plot">
          <Plot
            data={_.map<any, Partial<PlotData>>(
              rowsByNodeName,
              (rows, nodeName) => ({
                x: _.map(rows, row => row.time),
                y: _.map(rows, row => row.value),
                type: "scatter",
                mode: "lines+markers",
                name: nodeName,
                showlegend: true
              })
            )}
            onLegendClick={this.handleLegendClick}
            onClick={this.handlePointClick}
            layout={{
              width: 1000,
              height: 600,
              title: "Network Out All AVG",
              hovermode: "closest"
            }}
          />
        </div>
      </div>
    );
  }

  private handleLegendClick = (
    eventData: Readonly<Plotly.LegendClickEvent>
  ): boolean => {
    const nodeName = eventData.data[eventData.curveNumber].name;
    this.props.history.push(`/graph/${nodeName}`);
    return false;
  };

  private handlePointClick = (
    eventData: Readonly<Plotly.PlotMouseEvent>
  ): boolean => {
    const nodeName = eventData.points[0].data.name;
    this.props.history.push(`graph/${nodeName}`);
    return false;
  };

  private handleChangeFromTime = (date: moment.Moment) => {
    this.props.dispatch(
      changeNetworkOutAllAVGFilters({
        time: {
          fromTime: date.unix(),
          toTime: this.props.toTime
        }
      })
    );
  };
  private handleChangeFromTimeRawDate = (event: any) => {
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(
        changeNetworkOutAllAVGFilters({
          time: {
            fromTime: newDate.unix(),
            toTime: this.props.toTime
          }
        })
      );
    }
  };

  private handleChangeToTime = (date: moment.Moment) => {
    this.props.dispatch(
      changeNetworkOutAllAVGFilters({
        time: {
          fromTime: this.props.fromTime,
          toTime: date.unix()
        }
      })
    );
  };
  private handleChangeToTimeRawDate = (event: any) => {
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(
        changeNetworkOutAllAVGFilters({
          time: {
            fromTime: this.props.toTime,
            toTime: newDate.unix()
          }
        })
      );
    }
  };
}

const mapStateToProps = (state: ReducerConfigure) => ({
  data: state.graphReducer.networkOutAllAVGGraph.data,
  fromTime: state.graphReducer.networkOutAllAVGGraph.time.fromTime,
  toTime: state.graphReducer.networkOutAllAVGGraph.time.toTime
});

export default connect(mapStateToProps)(NetworkOutAllAVGGraph);
